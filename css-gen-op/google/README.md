# Generate the Google trace

Run this command from `css-gen-op/google/`:

```sh
python3 ../generate.py google.trace
```

The script reads the raw browser capture and writes DOM commands to `command.json` in the current directory.
