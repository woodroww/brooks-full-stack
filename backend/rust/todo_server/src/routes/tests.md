
# create user

curl -X POST \
localhost:3010/api/v1/users \
-H "Content-Type: application/json" \
--data '{ "username": "woodroww", "password": "myfancypass" }'

# login

curl -X POST \
localhost:3010/api/v1/users/login \
-H "Content-Type: application/json" \
--data '{ "username": "woodroww", "password": "myfancypass" }'

# logout

curl -X POST \
localhost:3010/api/v1/users/logout \
-H "x-auth-token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.Indvb2Ryb3d3Ig.8H93zyAZ9KqTTngPPAepYMrMLWRQBXcKsxRX1Jn8HxQ"

# create a task

curl -X POST \
localhost:3010/api/v1/tasks \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc2OTkyfQ.iEgWdomqYA3SkFZOiQmSvQPFLSW4kfsHVxA9p-WN8KA" \
-H "Content-Type: application/json" \
--data '{ "title": "Curl is fun", "description": "typing and stuff in the terminal" }'

# get a task

curl -X GET \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc4Mjg1fQ.KMLTPRSfhiKxfeVx4t1bF9VSUb7HsFOAZwwFcrtYLXE" \
localhost:3010/api/v1/tasks/8

# set task with id as completed

curl -X PUT \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc4Mjg1fQ.KMLTPRSfhiKxfeVx4t1bF9VSUb7HsFOAZwwFcrtYLXE" \
localhost:3010/api/v1/tasks/8/completed

# set task with id as uncompleted

curl -X PUT \
-H "x-auth-token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6Indvb2Ryb3d3IiwiaWF0IjoxNjUxODc4Mjg1fQ.KMLTPRSfhiKxfeVx4t1bF9VSUb7HsFOAZwwFcrtYLXE" \
localhost:3010/api/v1/tasks/8/uncompleted




