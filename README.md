# Description

This system is a ***data ingestor controller*** which is a sub-system needed to serve as a reliable intermediary between the user-facing 
applications and the dataset ingestors.

There are many data ingestors and and ingestors are not 
equal since each is skilled differently in dealing with one or more data source protocols.

In Simple words, a data ingestor controller is like a **demultiplexer**. 
A demultiplexer (or demux) is a device that takes a **single input line** *(in this case, an ingestion job with a specific data source protocol)* and ***routes*** it to **one of several digital output lines** *(in this case, the appropriate data ingestors)*

This system which acts as a Network Service for communication through a REST API performs 5 operations:
* Accept a DatasetIngestionJob and store it. The job has a unique ID, a user ID, an urgency boolean flag, a 
data-source protocol (jdbc|odbc|s3|looker), and a text query in a language accepted by the dataset source.
  
* Respond to a request from a dataset ingestor asking for a suitable job. The jobs handed out should be 
prioritized by their creation order, unless if they’re urgent, in which case urgent jobs have higher priority 
than non-urgent ones. Requests from ingestors should contain their advertised protocol capabilities, so 
ingestors should only be given jobs that they can handle.
  
* Accept a DatasetIngestionResult and store it. The result contains a unique ID, the job ID, the status 
of the execution of the job (success|failure), and the produced dataset.
  
* Respond to a request from a user-facing application asking for the ingestion status of a job it had 
provided earlier. It should produce a response with the ingestion status of either (successful|pending|failed).
  
* Respond to a request from an insight generation system asking for the dataset produced for a job, given 
its ID.

## Building, Testing, Running, and Connecting to the Service

To build, just clone the repo by running the following command in Git Bash in the appropriate directory:

``git clone https://github.com/eyadnawar/kausa-task-rust.git``


To conect to the service, there are 5 endpoints that correspond to each of the aforementioned operations. These endpoints are:

* POST `/create-job`
to be used by user-facing applications
Receives a ``DatasetIngestionJob``to be registered for consumpion by an ingestor.

Request Data:

    1. `user_id`: String                      ### The user ID. The relation between a user and an ingestion job is one-to-many
    2. `urgency`: Boolean                     ### wether or not the ingestion job is of urgent priority
    3. `data_source_protocol`: String         ### only one of (jdbc|odbc|s3|looker)
    4. `text_query`: String                   ### The query to be executed to retrieve the dataset
    5. `url`: String                          ### The url where the aforementioned query will be executed
       
Response Data:

    1. `job_id`                               ### The job id of the registered job to be used later for status and result retrieval
    2. `status`
    3. `mesage`

* GET `/request-job/<protocol>`
To be used by ingestors to request a job for execution. the <protocol> parameter is the requested job protocol. One of ["jdbc", "odbc", "s3", "looker"]

URL parameters:

    1. protocol                               ### String indicating the protocol of the requested job for ingestion

Response Data:    

    1. `user_id`: String                      ### The user ID. The relation between a user and an ingestion job is one-to-many
    2. `urgency`: Boolean                     ### wether or not the ingestion job is of urgent priority
    3. `data_source_protocol`: String         ### only one of (jdbc|odbc|s3|looker)
    4. `text_query`: String                   ### The query to be executed to retrieve the dataset
    5. `url`: String                          ### The url where the aforementioned query will be executed
    6. `job_id`: String
    7. `text_query`: String                   ### The query to be executed to retrieve the dataset
    8. `url`: String                          ### The url where the query will be executed to retrieve the dataset


* POST `/update-result` 
Set by a data ingestor to inform the former about the data ingestion result of a specific job.

Request Data:

    1. `job_id`: String
    2. `status`: String                      ### Success/failure
    3. `dataset_location`: String            ### uri for the dataset location on the disk (metadata)

Response Data:

In case of success the controller returns a string informing the requesting end that the job status has been updated


* GET `/get-status/<job_id>`
Sent to the controller by the user-facing application. The controller receives the `job_id` as a parameter in the url of the request, searches the database for the job with the matching `job_id`, and checks the ingestion status of that job. The controller responds with `Success/Failure/Pending`

URL parameters:

    1. `job_id`                              ### The id of the job whose status is requested

Response Data:

A string with the status of the job


* GET `/get-dataset/<job_id>`
Sent to the controller by the an ingestion generator application. The controller receives the `job_id` as a parameter in the url of the request, searches the database for the job with the matching `job_id`, and returns a string with the location (url) of th produced dataset

URL parameters:

    1. `job_id`                              ### The id of the job whose status is requested

Response Data:

A string with the location (metadata) of the job

## Technical *(Implementation)* Details

This system is built in `Rust` and uses `Rocket` v0.4.7 for building the REST api

The state of the data ingestor controller is stroed in the app's main memory. The state of the controller is represented by 2 queues *(urgent/non-urgent)* for each of the data source protocols that the controller supports `(jdbc|odbc|s3|looker)`
The controller delegates jobs to the appropriate data ingestors from the non-urgent queue only if the urgent queue is empty.

The 2 queues approach was better *efficiency-wise* than using 1 `queue` or 1 `Arraylist`. The reason is that an `ArrayList`
 would take `O(n)` for `insertion` and `O(n)` for `deletion`, whereas inserting and deleting from the queue is a cheap `O(1)` operation.
