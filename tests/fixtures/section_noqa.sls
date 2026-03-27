getpip:
  cmd.run:
    - name: /usr/bin/python /usr/local/sbin/get-pip.py
    - quiet # noqa 901

/etc/http/conf/http.conf:
  file.managed:
    - template: jinja
    - context:  # noqa 219
      custom_var: "override"
