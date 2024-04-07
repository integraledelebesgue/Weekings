import pymongo
from db import get_client
from dotenv import load_dotenv
import os

# Load environment variables
load_dotenv()

db_name = os.getenv("MONGODB_DBNAME")
client = get_client()
db = client[db_name]
categories = db["Categories"]


get_category = lambda name: categories.find_one({"name": name})


def insert_categories():
    while True:
        name = input("Enter a category name: ")
        if name == "":
            break

        parent_name = input("Enter the parent category (if any): ")
        
        if get_category(name):
            print("Category already exists.")
            continue

        parent = get_category(parent_name) if parent_name else None
        if not parent and parent_name:
            print("Parent category does not exist.")
            continue

        parent_id = parent["_id"] if parent else None
        category = {
            "name": name,
            "subcategories": [],
            "supercategory": parent_id
        }
        categories.insert_one(category)

        if not parent:
            print("Category added successfully.")
            continue

        parent_subcategories = parent["subcategories"]
        parent_subcategories.append(category["_id"])
        categories.update_one({"_id": parent_id}, {"$set": {"subcategories": parent_subcategories}})
        print("Category added successfully.")


def delete_categories():
    while True:
        name = input("Enter a category name to delete: ")
        if name == "":
            break

        category = get_category(name)
        if not category:
            print("Category does not exist.")
            continue
        if category["subcategories"]:
            print("Category has subcategories. Please delete them first.")
            continue

        parent_id = category["supercategory"]
        parent = categories.find_one({"_id": parent_id})
        parent_subcategories = parent["subcategories"]
        parent_subcategories.remove(category["_id"])
        categories.update_one({"_id": parent_id}, {"$set": {"subcategories": parent_subcategories}})
        categories.delete_one({"_id": category["_id"]})
        print("Category deleted successfully.")


def print_categories():
    print("Categories:")
    for category in categories.find():
        print(category)


def main():
    while True:
        action = input("What do you want from me??? (add/del/ls/q): ")
        if action == "":
            return
        if action == "add":
            insert_categories()
        elif action == "del":
            delete_categories()
        elif action == "ls":
            print_categories()
        elif action == "q":
            break
        else:
            print("Invalid action. Try again.")


if __name__ == "__main__":
    main()
