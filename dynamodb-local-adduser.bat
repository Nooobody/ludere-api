aws dynamodb put-item ^
  --table-name Users ^
  --item "{ \"username\": {\"S\": \"Asdf\"}, \"password\": {\"S\": \"$argon2i$v=19$m=16,t=2,p=1$YXNkZmFzZGY$AYo4g2O8+H79T1Z/rQQ7Lg\"}}" ^
  --endpoint-url http://localhost:8000
