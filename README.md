# Usage

1. Create a ALB in AWS.
2. Make it point to a lambda called `rust-web` (needed by deploy.sh)
3. ./deploy.sh

Test with:

    $ echo '{"name": "Matthew", "age": 3000}' | http 'lambda-test-898061068.ap-southeast-2.elb.amazonaws.com'
    HTTP/1.1 200 OK
    Connection: keep-alive
    Content-Length: 33
    Content-Type: application/octet-stream
    Date: Mon, 06 Sep 2021 11:34:13 GMT
    Server: awselb/2.0

    Hello Matthew who is 3000 years old
