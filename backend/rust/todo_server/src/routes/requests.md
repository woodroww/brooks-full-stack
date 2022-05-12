helpful webpage
https://reqbin.com/req/c-d2nzjn3z/curl-post-body


testing todo app
cypress.io download now
in ./integration-tests/frontend-tests run npm install

docker-compose --profile js-vue up

docker ps
find local port on js-vue
change port in ./integration-tests/frontend-tests/cypress.json to what docker says
for docker js-vue
  "baseUrl": "http://localhost:57856"
to use the app I am working on use this
  "baseUrl": "http://localhost:8080"

## routes
/Users/matt/external_code/BrooksYew/brooks-full-stack/backend/nodejs/express/routes/users.js


# create user
## route: "/" 

curl -X POST \
localhost:3010/api/v1/users \
-H "Content-Type: application/json" \
--data '{ "username": "woodroww", "password": "myfancypass" }'

### response:
{
    "data": {
        "id": 3,
        "username": "woodroww",
        "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc1ODYzfQ.8CFt61SbF0J7QxpVTYSzfatIrWaAUM8CK_iedXzTjqo"
    }
}


# login
## route: "/login" 

curl -X POST \
localhost:3010/api/v1/users/login \
-H "Content-Type: application/json" \
--data '{ "username": "woodroww", "password": "myfancypass" }'

### response:
(with different token than from the creation request)
{
	"data": {
		"id": 3,
		"username": "woodroww",
		"token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc4Mjg1fQ.KMLTPRSfhiKxfeVx4t1bF9VSUb7HsFOAZwwFcrtYLXE"
	}
}


# logout
## route: "/logout"

curl -X POST \
localhost:3000/api/v1/users/logout \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc2MjQxfQ.dFoUWNAMpiiyXC2lKDsU_tZ88Kvb-lIFOf9_8QEzg9E"

### response:
{"message":"user logged out"}


# create a task
## route: "/" POST

curl -X POST \
localhost:3000/api/v1/tasks \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc2OTkyfQ.iEgWdomqYA3SkFZOiQmSvQPFLSW4kfsHVxA9p-WN8KA" \
-H "Content-Type: application/json" \
--data '{ "title": "Curl is fun", "description": "typing and stuff in the terminal" }'

### response:
{
	"data": {
		"id": 8,
		"priority": null,
		"title": "Curl is fun",
		"completed_at": null,
		"description": "typing and stuff in the terminal"
	}
}


# get all a users tasks
## route: "/" GET
curl -X GET \
localhost:3000/api/v1/tasks \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc2OTkyfQ.iEgWdomqYA3SkFZOiQmSvQPFLSW4kfsHVxA9p-WN8KA" \


# get a task
## route: "/:taskId" GET

curl -X GET \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc4Mjg1fQ.KMLTPRSfhiKxfeVx4t1bF9VSUb7HsFOAZwwFcrtYLXE" \
localhost:3000/api/v1/tasks/8

### response:
{
	"data": {
		"id": 8,
		"priority": null,
		"title": "Curl is fun",
		"completed_at": null,
		"description": "typing and stuff in the terminal",
		"deleted_at": null,
		"user_id": 3,
		"is_default": false
	}
}


# update a task
## route: "/:taskId" PATCH

curl --location --request PATCH 'http://localhost:3010/api/v1/tasks/253' \
--header 'x-auth-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.InVuaXF1ZV9uYW1lMSI.udg0H21G8eVyG8fO4fr2jisFtz4KtV_TEUIV3HMNQbk' \
--header 'Content-Type: application/json' \
--data-raw '{
        "id": 253,
        "priority": "A",
        "title": "Curpatching suxl is fun",
        "completed_at": null,
        "description": "dame caca is not super fun typing and stuff in the terminal"
}'

curl --location --request PATCH 'http://localhost:3010/api/v1/tasks/253' \
--header 'x-auth-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.InVuaXF1ZV9uYW1lMSI.udg0H21G8eVyG8fO4fr2jisFtz4KtV_TEUIV3HMNQbk' \
--header 'Content-Type: application/json' \
--data-raw '{
        "id": 253,
        "priority": "A",
        "title": "Curpatching suxl is fun",
        "completed_at": "2022-05-11T18:45:16.123456",
        "description": "dame caca is not super fun typing and stuff in the terminal"
}'

# delete a task
## route: "/:taskId" DELETE

# set task with id as completed
## route: "/:taskId/completed"

curl -X PUT \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc4Mjg1fQ.KMLTPRSfhiKxfeVx4t1bF9VSUb7HsFOAZwwFcrtYLXE" \
localhost:3000/api/v1/tasks/8/completed

### response:
OK


# set task with id as uncompleted
## route: "/:taskId/uncompleted"

curl -X PUT \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc4Mjg1fQ.KMLTPRSfhiKxfeVx4t1bF9VSUb7HsFOAZwwFcrtYLXE" \
localhost:3000/api/v1/tasks/8/uncompleted

### response:
OK



