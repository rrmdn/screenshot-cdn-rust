## Screenshot CDN

A simple Rust application that lets you take screenshots of any web pages.

### Usage

Run this image via docker

```shell
docker run -d -p 8080:8080 rrmdn/screenshot-cdn-rust
```

Once running, try opening the following url in your browser to take a screenshot of google.com

```
http://localhost:8080/v1/screenshot?url=https://google.com
```

It will respond with a screenshot of google.com in a JPEG file with an expiration header of 4 weeks so you can minimize the load by proxying this endpoint with a CDN. Expiration will be configurable in the near future.

Available options:

| option  | required | default   | description                             |
| ------- | -------- | --------- | --------------------------------------- |
| url     | yes      | -         | the web page url                        |
| quality | no       | 50        | JPEG quality, 0-100                     |
| delay   | no       | 0         | screenshot delay after DOM ready, in ms |
| device  | no       | Galaxy S5 | device emulation                        |

Available emulation devices can be found [here](https://github.com/puppeteer/puppeteer/blob/main/src/common/DeviceDescriptors.ts#L33).