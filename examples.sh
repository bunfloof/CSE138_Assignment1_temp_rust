set -x

curl --request GET --header "Content-Type: application/json" --write-out "\n%{http_code}\n" http://localhost:8090/hello
#{"message":"world"}
#200

curl --request POST --write-out "\n%{http_code}\n" http://localhost:8090/hello
#Method Not Allowed
#405

curl --request POST --write-out "\n%{http_code}\n" http://localhost:8090/hello/slug
#{"message":"Hi, slug."}
#200

curl --request GET --write-out "\n%{http_code}\n" http://localhost:8090/hello/slug
#Method Not Allowed
#405

curl --request GET --header "Content-Type: application/json" --write-out "\n%{http_code}\n" http://localhost:8090/test
# {"message":"test is successful"}
# 200

curl --request POST --header "Content-Type: application/json" --write-out "\n%{http_code}\n" "http://localhost:8090/test?msg=foo"
# {"message":"foo"}
# 200

curl --request POST --write-out "\n%{http_code}\n"  http://localhost:8090/test
# Bad Request
# 400

