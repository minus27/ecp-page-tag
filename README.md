# Edge Compute JavaScript Page Tagging Example

## Overview
This Fastly Compute@Edge project demonstrates how HTML content in a response body can be intercepted and modified for the purposes of "Page Tagging" or adding a JavaScript `<script>` element.

This project is based on the Compute@Edge default starter kit for Rust that demonstrates routing, simple synthetic responses, and overriding caching rules.

**For more details about this and other starter kits for Compute@Edge, see the [Fastly developer hub](https://developer.fastly.com/solutions/starters)**


## Project Details
### Backend Application
This Compute@Edge frontends a Fastly VCL Service that, among other things, returns the name of the Fastly POP that a request is routed to when is is *directly* accessed:

```
$ curl -sw"\n" https://info.demotool.site/pop
EWR
$
```

If you are using a browser, or set your Accept header to "text/html", the response is wrapped in some HTML, which is needed for the page tagging to work:

```
$ curl -sw"\n" https://info.demotool.site/pop -H "accept: text/html"
<!DOCTYPE html><html><head><title>info.demotool.site</title></head><body><pre style="word-wrap: break-word; white-space: pre-wrap;">EWR</pre><script type="text/javascript">function sortObj(o){return Object.keys(o).sort().reduce(function(x,k){x[k]=o[k];return x;},{});}try{document.querySelector('pre').innerText=JSON.stringify(sortObj(JSON.parse(document.querySelector('pre').innerText)),null,'  ');}catch(e){console.log(`Error: ${e}`);}</script></body></html>
$
```

### Functionality instrumented using Compute@Edge
When this Service is *indirectly* through the Compute@Edge Service for this project, the following occurs:

- the requested content, e.g. `/pop`, from info.demotool.site is fetched
- a `<script src="invert_colors_script.js" defer></script>` script tag is appended to the `<head>` element
- a `X-Toml-Version` response header with the Service version is added
- the modified response is returned

In addition, for the sake of simplicity, the Compute@Edge Service for this project returns the JavaScript code for the `invert_colors_script.js` script:

```
document.querySelector('body').setAttribute('style','color:white; background-color:black;')
```

### Final Result
Here is the output when the original Fastly Service is *indirectly* accessed through the Compute@Edge Service for this project showing the addition of the JavaScript `<script>` element:

```
$ curl -sw"\n" https://page-tag.edgecompute.app/pop -H "accept: text/html"
<!DOCTYPE html><html><head><title>info.demotool.site</title><script src="invert_colors_script.js" defer></script></head><body><pre style="word-wrap: break-word; white-space: pre-wrap;">EWR</pre><script type="text/javascript">function sortObj(o){return Object.keys(o).sort().reduce(function(x,k){x[k]=o[k];return x;},{});}try{document.querySelector('pre').innerText=JSON.stringify(sortObj(JSON.parse(document.querySelector('pre').innerText)),null,'  ');}catch(e){console.log(`Error: ${e}`);}</script></body></html>
$
```


