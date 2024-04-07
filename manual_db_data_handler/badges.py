# Badge example:
# _id: 6611b0cdf6a8e04dcc4fbe82
# name: "First Art Post"
# description: "New artirst has joined the game!"
# category: 6611afeaf6a8e04dcc4fbe81
# threshold: 1
from db import get_client
from dotenv import load_dotenv
import os

# Load environment variables
load_dotenv()

db_name = os.getenv("MONGODB_DBNAME")
client = get_client()
db = client[db_name]

badges = db["Badges"]
categories = db["Categories"]

get_badge = lambda name: badges.find_one({"name": name})


def insert_badges():
    while True:
        name = input("Enter a badge name: ")
        if name == "":
            break

        description = input("Enter a description: ")
        category_name = input("Enter the category name: ")
        threshold = int(input("Enter the threshold: "))
        
        if get_badge(name):
            print("Badge already exists.")
            continue

        category = categories.find_one({"name": category_name})
        if not category:
            print("Category does not exist.")
            continue

        badge = {
            "name": name,
            "description": description,
            "category": category["_id"],
            "threshold": threshold
        }
        badges.insert_one(badge)
        print("Badge added successfully.")


def delete_badges():
    while True:
        name = input("Enter a badge name to delete: ")
        if name == "":
            break

        badge = get_badge(name)
        if not badge:
            print("Badge does not exist.")
            continue

        badges.delete_one({"_id": badge["_id"]})
        print("Badge deleted successfully.")


def print_badges():
    for badge in badges.find():
        print(badge)


def main():
    while True:
        action = input("What would you like to do? (add/del/ls/q)")
        if action == "add":
            insert_badges()
        elif action == "del":
            delete_badges()
        elif action == "ls":
            print_badges()
        elif action == "q":
            break
        else:
            print("Invalid action. Try again.")

if __name__ == "__main__":
    main()