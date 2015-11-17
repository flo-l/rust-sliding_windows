var searchIndex = {};
searchIndex['sliding_windows'] = {"items":[[0,"","sliding_windows","This crate provides an iterator adaptor that yields sliding windows into the elements of the wrapped Iterator.",null,null],[3,"SlidingWindowStorage","","This holds the backing allocation for the `Window` of a `SlidingWindowAdaptor`.",null,null],[3,"SlidingWindowAdaptor","","See [sliding_windows](index.html) for more information.",null,null],[11,"new","","Create a new `SlidingWindowStorage` with a given window size.\nThis will allocate as much memory as is needed to store the Window automatically.",0,{"inputs":[{"name":"slidingwindowstorage"},{"name":"usize"}],"output":{"name":"slidingwindowstorage"}}],[11,"from_vec","","Create a new `SlidingWindowStorage` with a given window size from a given `Vec`.\nThe contents of the Vec will be removed.\nThis will reuse the allocation of the Vec instead of allocating new memory.",0,{"inputs":[{"name":"slidingwindowstorage"},{"name":"vec"},{"name":"usize"}],"output":{"name":"slidingwindowstorage"}}],[11,"into","","",0,{"inputs":[{"name":"slidingwindowstorage"}],"output":{"name":"vec"}}],[11,"new","","This creates a new SlidingWindowAdaptor. Usually you should be using",1,{"inputs":[{"name":"slidingwindowadaptor"},{"name":"i"},{"name":"slidingwindowstorage"}],"output":{"name":"slidingwindowadaptor"}}],[11,"next","","",1,{"inputs":[{"name":"slidingwindowadaptor"}],"output":{"name":"option"}}],[8,"IterExt","","",null,null],[11,"sliding_windows","","",2,{"inputs":[{"name":"iterext"},{"name":"slidingwindowstorage"}],"output":{"name":"slidingwindowadaptor"}}]],"paths":[[3,"SlidingWindowStorage"],[3,"SlidingWindowAdaptor"],[8,"IterExt"]]};
initSearch(searchIndex);
