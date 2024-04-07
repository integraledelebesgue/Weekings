from pymongo.mongo_client import MongoClient
from dotenv import load_dotenv
import os

# Load environment variables
load_dotenv()

# Get the MongoDB connection URI from the environment variable
user = os.getenv("MONGODB_USER")
password = os.getenv("MONGODB_PASSWORD")
dbname = os.getenv("MONGODB_DBNAME")

uri = f"mongodb+srv://{user}:{password}@weekings.jpw1wfw.mongodb.net/?retryWrites=true&w=majority&appName=Weekings"

def get_client():
    client = MongoClient(uri)
    try:
        client.admin.command('ping')
        print("Pinged your deployment. You successfully connected to MongoDB!")
    except Exception as e:
        print(e)
        raise ValueError("Failed to connect to MongoDB. Check your `uri`")

    return MongoClient(uri)
