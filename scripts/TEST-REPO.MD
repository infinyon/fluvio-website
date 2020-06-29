## Test Repo Script Step-by-Step

1. Clone a branch to `.tmp` directory

    ```
    git clone --single-branch --branch scripts https://github.com/infinyon/fluvio-cloud.git .tmp/fluvio-cloud-123
    ```

2. detach from `origin`

    ```
    cd .tmp/fluvio-cloud-123
    git remote rm origin
    ```

3. copy directory content into their target directories

    ```
    cp -rf website/docs ../../content/docs/fluvio-cloud
    cp -rf website/img ../../static/img/fluvio-cloud
    ```

4. clean-up

    ```
    rm -rf .tmp/fluvio-cloud-123
    ```