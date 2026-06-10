
const uiapi_path = "/uiapi/";
/**
 * 
 * @param {string} str 
 */
function uiapi_path_join(str){
    if(str.startsWith("/")){
        str = str.substring(1);
    }

    return uiapi_path + str;
}

/**
 * 
 * @param {any} param 
 */
async function callUiApi(param){
    let res =  await fetch(uiapi_path,{
        method : "POST",
        body : JSON.stringify(param)
    });

    let txt = await res.text();
    return txt;
} 


async function uiLog(dd){
    let f = await fetch(uiapi_path_join("printlog"),{
        method : "POST",
        body :dd
    });
}

callUiApi().then(e=>{
    uiLog("halo dunia ini dari ui");
})