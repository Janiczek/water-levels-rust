```
curl https://water-levels.herokuapp.com/rain \
  -H 'Content-Type: application/json' \
  --data-raw '{"landscape": [3,1,1,1,1,1,1,1,1,1,6,4,5,6], "hours": 3}'


# --> {"result":[5.1,5.1,5.1,5.1,5.1,5.1,5.1,5.1,5.1,5.1,6.0,6.0,6.0,6.0]}
```
