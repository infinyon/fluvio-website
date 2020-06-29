## Test Repo Script Step-by-Step

1. Clone a branch to `.tmp` directory

    ```
    git clone --single-branch --branch <branch-name> https://github.com/infinyon/<project-name>.git .tmp/<project-name>-<timestamp>
    ```

2. detach from `origin`

    ```
    cd .tmp/<project-name>-<timestamp>
    git remote rm origin
    ```

3. copy contents into their target directories (content is swapped - rather than augmented)

    ```
    cp -rf website/docs ../../content/docs/<project-name>
    cp -rf website/img ../../static/img/<project-name>
    ```

4. clean-up

    ```
    rm -rf .tmp/<project-name>-<timestamp>
    ```