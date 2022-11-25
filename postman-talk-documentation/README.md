# API Documentation

## Route: "/"

### Get
URL: **/**  
Get HTML back that says that the API is running  
  
------------------------------------
## Route: "/hello"
  
### Get
URL: **/hello**  
Get Plaintext "Hello, world!" back  
  
------------------------------------
## Route: "/data"
  
### Get
Authentication needed via API Key in Header  
URL: **/data**  
Header: **x-api-key**  
Value: **valid-api-key**  
Returns all saved file IDs and their respective FileName  
  
### Get
URL: **/data/\<id\>**  
Returns data saved in file with id  
  
### Post with query and JSON Body
Authentication needed via API Key in Header  
URL: **/data?\<name\>&\<id\>**  
Header: **x-api-key**  
Value: **valid-api-key**  
Body: **application/json**  
Saves the JSON Body value as file with \<name\> and \<id\>  
  
### Patch
Authentication needed via API Key in Header  
URL: **/data/\<id\>**
Header: **x-api-key**  
Value: **valid-api-key**  
Body: **application/json**  
Overwrites saved data with \<id\> with the body of the request  
  
### Delete with ID
Authentication needed via API Key in Header  
URL: **/data/\<id\>**  
Header: **x-api-key**  
Value: **valid-api-key**  
Body: **application/json**  
Deletes File with \<id\>

### Delete
Authentication needed via API Key in Header  
URL: **/data**  
Header: **x-api-key**  
Value: **valid-api-key**  
Body: **application/json**  
Deletes all saved data
