
aws dynamodb create-table ^
  --table-name Users ^
  --attribute-definitions AttributeName=username,AttributeType=S ^
  --key-schema AttributeName=username,KeyType=HASH ^
  --endpoint-url http://localhost:8000 ^
  --billing-mode PROVISIONED ^
  --provisioned ReadCapacityUnits=1,WriteCapacityUnits=1
