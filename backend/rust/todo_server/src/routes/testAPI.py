import requests
import json
import random
import string
import pytest


def create_user(user, password):
    header = { "Content-Type": "application/json" }
    payload = { "username": user, "password": password }
    r = requests.post(
        "http://localhost:3010/api/v1/users",
        data=json.dumps(payload),
        headers=header)
    assert(r.status_code == requests.codes.ok)
    json_response = r.json()["data"]
    print("create_user() data")
    #print(json_response)
    print(json.dumps(json_response, indent=1))
    json_response["id"]
    json_response["username"]
    jwt = json_response["token"]
    print("create_user() passed")
    return jwt


def login(user, password):
    header = { "Content-Type": "application/json" }
    payload = { "username": user, "password": password }
    r = requests.post(
        "http://localhost:3010/api/v1/users/login",
        data=json.dumps(payload),
        headers=header)
    assert(r.status_code == requests.codes.ok)
    json_response = r.json()["data"]
    print("login() data")
    print(json.dumps(json_response, indent=1))
    #print(json_response)
    json_response["id"]
    json_response["username"]
    jwt = json_response["token"]
    print("login() passed")
    return jwt, json_response["id"]


def logout(jwt):
    header = { "x-auth-token": jwt }
    r = requests.post(
        "http://localhost:3010/api/v1/users/logout",
        headers=header)
    assert(r.status_code == requests.codes.ok)
    json_response = r.json()["message"]
    print(json.dumps(json_response, indent=1))
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
    print("create_task() data")
    print(json.dumps(json_response, indent=1))
    #print(json_response)
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
    print("get_task() data")
    print(json.dumps(json_response, indent=1))
    #print(json_response)
    print("get_task() passed")


def get_all_user_tasks(jwt, user_id):
    header = { "x-auth-token": jwt }
    r = requests.get(
        f"http://localhost:3010/api/v1/tasks",
        headers=header)
    print(f"get_all_user_tasks() {r.status_code}")
    assert(r.status_code == requests.codes.ok)
    json_response = r.json()
    print(json.dumps(json_response, indent=1))
    print("get_all_user_tasks() passed")


def update_task(jwt, task_id):
    header = { "Content-Type": "application/json", "x-auth-token": jwt }
    payload = {
        "id": task_id,
        "priority": 'A',
        "title": "Hambones",
        "completed_at": "2022-05-11T18:45:16.214145",
        "description": "abcdefghijklmnopqrstuvwxyz"
    }
    print(f"update_task() sending:\n {json.dumps(payload)}")
    r = requests.patch(
        f"http://localhost:3010/api/v1/tasks/{task_id}",
        data=json.dumps(payload),
        headers=header)
    print(f"update_task() {r.status_code}")
    assert(r.status_code == requests.codes.ok)
    json_response = r.json()
    print("update_task response:")
    print(json.dumps(json_response, indent=1))
    json_response = json_response["data"]
    assert(json_response["id"] == payload["id"])
    assert(json_response["priority"] == payload["priority"])
    assert(json_response["title"] == payload["title"])
    assert(json_response["completed_at"] == payload["completed_at"])
    assert(json_response["description"] == payload["description"])
    print("update_task() passed")


def mark_task_complete(jwt, task_id):
    header = { "x-auth-token": jwt }
    r = requests.put(
        f"http://localhost:3010/api/v1/tasks/{task_id}/completed",
        headers=header)
    assert(r.status_code == requests.codes.ok)
    print("mark_task_complete() passed")


def mark_task_uncompleted(jwt, task_id):
    header = { "x-auth-token": jwt }
    r = requests.put(
        f"http://localhost:3010/api/v1/tasks/{task_id}/uncompleted",
        headers=header)
    assert(r.status_code == requests.codes.ok)
    print("mark_task_uncompleted() passed")


def test_main():
    letters = string.ascii_letters
    user = "".join(random.choices(letters, k=10))
    password = "".join(random.choices(letters, k=10))
    create_user(user, password)
    jwt, user_id = login(user, password)

    title = "".join(random.choices(letters, k=10))
    description = "".join(random.choices(letters, k=10))
    task_id = create_task(jwt, title, description)
    for i in range(5):
        create_task(jwt, f"{title}_{i}", f"{description}_{i}")

    get_all_user_tasks(jwt, user_id)
    get_task(jwt, task_id)
    mark_task_complete(jwt, task_id)
    mark_task_uncompleted(jwt, task_id)
    update_task(jwt, task_id)
    logout(jwt)



if __name__ == "__main__":
    test_main()
