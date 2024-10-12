# Diff
## /out.js
### esbuild
```js
// Users/user/project/node_modules/d/index.js
var foo = 123;

// Users/user/project/node_modules/b1/index.js
throw "keep this 1";

// Users/user/project/node_modules/b2/index.js
throw "keep this 2";

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
@@ -1,4 +0,0 @@
-var foo = 123;
-throw "keep this 1";
-throw "keep this 2";
-console.log(foo);

```