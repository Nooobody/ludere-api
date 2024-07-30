aws dynamodb get-item ^
  --table-name Users ^
  --key "{ \"username\": {\"S\": \"Asdf\"}}" ^
  --endpoint-url http://localhost:8000
