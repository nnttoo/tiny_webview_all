// @ts-check
/**
 * 
 * @type {import(".").FetchFun} 
 */
const reqApi = async (p) => {
    let restResult = await fetch("/controlwindow", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(p),
    });

    let resultTxt = await restResult.text();
    return resultTxt;
}

let isMax = false;

function intButton() {
    /** 
     * @type {import(".").FunBtn} 
     */
    function btn(selector) {
        //@ts-ignore
        return document.querySelector(selector);
    }

    /**
     *  @type {import(".").FunInput} 
     */
    function ipt(selector) {
        //@ts-ignore
        return btn(selector);
    }


    btn("#close").onclick = () => {
        reqApi({
            cmd: "close",
            params: {}
        });
    }

    btn("#move").onclick = () => {

        reqApi({
            cmd: "move",
            params: {
                left: 100,
                top: 200
            }
        });
    }

    btn("#resize").onclick = () => {
        reqApi({
            cmd: "resize",
            params: {
                width: 800,
                height: 500
            }
        });
    }
    btn("#max").onclick = () => {
        isMax = !isMax;
        reqApi({
            cmd: "maximize",
            params: isMax
        });
    }

    btn("#min").onclick = () => {
        reqApi({
            cmd: "minimize",
            params: true
        });
    }
    btn("#openfdialog").onclick = async () => {
        let apiresult = await reqApi({
            cmd: "openfile",
            params: true
        });

        ipt("#ipt_dialog").value = apiresult;
    }
    btn("#openfolder").onclick = async () => {
        let apiresult = await reqApi({
            cmd: "openfolder",
            params: true
        });

        ipt("#ipt_dialog_folder").value = apiresult;
    }

}

intButton();