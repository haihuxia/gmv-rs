## gmv-rs
Move git projects to gitlab

### Usage
```shell script
cargo run -- config.yml
```

**console output:**

```
git: abc.git, msg: "repository \'abc.git\' does not exist", git: ""
git: http://ip:port/demo/demo-rest.git, msg: SUCCESS, new_git: http://ip:port/demo/demo-rest.git
```

**config.yml**

```
from:
  git:
    - abc.git
    - http://ip:port/demo/demo-rest.git
to:
  url: http://ip:port
  personal_token: fqz6DaAZxT9hikERzwed
  group: demo
```