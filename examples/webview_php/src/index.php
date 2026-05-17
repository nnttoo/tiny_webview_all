<?php 
/**
 * @param array{judul: string, status: string,callback : callable(string) :void} $arg
 */
function testDulu(array $arg){
    echo $arg["judul"];

}


testDulu(
    arg: [

    ]
);
testDulu([
    "judul" => "ini test dulu",
    "status" => "ini status", 
    "callback" => function(string $hasil){

    }
])
?>