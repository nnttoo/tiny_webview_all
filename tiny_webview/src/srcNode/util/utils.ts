
export function sleep(n : number){
    return new Promise((r,x)=>{
        setTimeout(()=>{r(null)}, n)
    });
}


export function createCBuffer(str: string): Buffer {
    return Buffer.from(str + "\0", "utf8");
}