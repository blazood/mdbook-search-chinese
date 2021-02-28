"use strict";
(function(){

    if(!elasticlunr){
        return;
    }

    elasticlunr.Pipeline.registerFunction(function(token) {
        if (token === null || token === undefined || token === '') {
            throw new Error('token should not be undefined');
        }
        return token;
    }, "trimmer-zh");

    let zh_stop_word_filter = [
        "的", "了"
    ]

    elasticlunr.Pipeline.registerFunction(function(token) {
        if (token && zh_stop_word_filter.indexOf(token) <= -1) {
            return token;
        }
    }, "stopWordFilter-zh");

    elasticlunr.Pipeline.registerFunction(function(token) {
        if (token === null || token === undefined || token === '') {
            throw new Error('token should not be undefined');
        }
        return token;
    }, "stemmer-zh");


})();