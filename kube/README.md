# About

How do we make this available in a Kube container ? We need to first build an image,
this image needs to run in a privileged pod mounting the node's `/sys/kernel/debug`
directory inside the pod.


# Building and running the image locally

As root:

```
$ podman build -t signals .
$ podman run -it --privileged --rm -v /sys/kernel/debug:/sys/kernel/debug localhost/signals
```
