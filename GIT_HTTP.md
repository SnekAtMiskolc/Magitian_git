# The basics of the git http protocol

### Notes
1. ```/``` in requests means the repositories path.

Let's say you want to push to a git repository at ***git.example.com/repo***.
Upon entering the command: 
```
git push
``` 
git will send the following request:
```
GET /info/refs?service=git-upload-pack
```

This will tell our server that we want to get the references (branches, tags ...) the server internally runs the following command: 
```
git-upload-pack --stateless-rpc --advertise-refs ${repository_path}
```
or
```
git upload-pack --stateless-rpc --advertise-refs ${repository_path}
```

If that request succeeds, git will continue by sending the following request:
```
POST /git-upload-pack
```
The post request contains a so called packfile which uses the pack protocol to tell the repository what things need to be added to the ODB (Object Database).

# Summary
What happened here?
First the server told us what it has then we saw that it was missing some things that we had or if it has everything.
In case it is missing some things we send it the missing objects as a packfile else we just quit and tell the user that everything is up to date.