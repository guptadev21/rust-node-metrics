from flask import Flask, jsonify
from pymongo import MongoClient

app = Flask(__name__)

# MongoDB connection setup
client = MongoClient('mongodb://localhost:27017/')
db = client['metrics_db']  # Replace 'your_database' with your database name
collection = db['metrics']  # Replace 'your_collection' with your collection name

@app.route('/')
def index():
    # Fetch data from MongoDB
    data = list(collection.find({}))
    
    # Convert MongoDB data to JSON serializable format
    for record in data:
        record['_id'] = str(record['_id'])  # Convert ObjectId to string
    
    return jsonify(data)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
