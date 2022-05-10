import requests
import json
import random
import string


def create_user(user, password):
    header = { "Content-Type": "application/json" }
    payload = { "username": user, "password": password }
    r = requests.post(
        "http://localhost:3010/api/v1/users",
        data=json.dumps(payload),
        headers=header)

    assert(r.status_code == requests.codes.ok)

    json_response = r.json()["data"]
    json_response["id"]
    json_response["username"]
    jwt = json_response["token"]
    print("create_user() passed")
    return jwt


def login():
    header = { "Content-Type": "application/json" }
    payload = { "username": user, "password": password }
    r = requests.post(
        "http://localhost:3010/api/v1/users/login",
        data=json.dumps(payload),
        headers=header)

    assert(r.status_code == requests.codes.ok)

    json_response = r.json()["data"]
    json_response["id"]
    json_response["username"]
    jwt = json_response["token"]
    print("login() passed")
    return jwt


def logout(jwt):
    header = { "x-auth-token": jwt }
    r = requests.post(
        "http://localhost:3010/api/v1/users/logout",
        headers=header)
    
    assert(r.status_code == requests.codes.ok)
    json_response = r.json()["message"]
    assert(json_response == "user logged out")
    print("logout() passed")



def create_task(jwt, title, description):
    header = { "Content-Type": "application/json", "x-auth-token": jwt }
    payload = { "title": title, "description": description }
    r = requests.post(
        "http://localhost:3010/api/v1/tasks",
        data=json.dumps(payload),
        headers=header)

    assert(r.status_code == requests.codes.ok)
    json_response = r.json()["data"]
    assert(json_response["title"] == title)
    print("create_task() passed")
    return json_response["id"]


def get_task(jwt, task_id):
    header = { "x-auth-token": jwt }
    r = requests.get(
        f"http://localhost:3010/api/v1/tasks/{task_id}",
        headers=header)
    
    assert(r.status_code == requests.codes.ok)
    json_response = r.json()["data"]
    print("get_task() passed")




letters = string.ascii_letters
user = "".join(random.choices(letters, k=10))
password = "".join(random.choices(letters, k=10))
create_user(user, password)
jwt = login()
title = "".join(random.choices(letters, k=10))
description = "".join(random.choices(letters, k=10))
task_id = create_task(jwt, title, description)
get_task(jwt, task_id)
logout(jwt)


