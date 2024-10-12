# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/demo-pkg/foo.js
console.log("foo");

// Users/user/project/node_modules/demo-pkg/bar/index.js
console.log("bar");
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out.js
+++ rolldown	
@@ -1,2 +0,0 @@
-console.log("foo");
-console.log("bar");

```