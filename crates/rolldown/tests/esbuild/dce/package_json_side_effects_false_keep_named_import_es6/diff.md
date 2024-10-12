# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/demo-pkg/index.js
var foo = 123;
console.log("hello");

// Users/user/project/src/entry.js
console.log(foo);
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out.js
+++ rolldown	
@@ -1,3 +0,0 @@
-var foo = 123;
-console.log("hello");
-console.log(foo);

```