import * as esbuild from 'esbuild'
import path from 'node:path'; 

async function getESBUildOption(): Promise<esbuild.BuildOptions> {
  
    return {
        entryPoints: [path.join(__dirname, "../src/index.ts")],
        bundle: true,
        outfile: path.join(__dirname,"../dist/lib/app.js"),
        platform: 'node',  // Agar sesuai dengan Node.js
        sourcemap: false,
        target: 'es6',
        minify: true,
        format: 'cjs',
        external: [],
        define: {
            __BUILD_DATE__: JSON.stringify(new Date().toLocaleString()),
            __ISDEV__: JSON.stringify(false),
            __ISSERVER__: JSON.stringify(true)
        },
        plugins: []
    }
}

export async function buildServer() {
    let buildOption = await getESBUildOption();
    await esbuild.build(buildOption);

    console.log('🖥️ Backend build sukses!');
}


async function buildServerWatch() {
    console.log("\n\nStart Server Build Watch");
    let buildOption = await getESBUildOption();

    if(buildOption.plugins == null) return;

    buildOption.plugins.push({
        name: 'rebuild-notify',
        setup(build) {
            build.onEnd(result => {
                console.log("🖥️ after build server watch");
            })
        },
    })

    const ctx = await esbuild.context(buildOption);
    await ctx.watch();
}


// async function build() {
 

//     if (process.argv.includes("--watch")) {
//         console.log("ini build browserwatch");
//         await buildServerWatch();
//         return;
//     } else {
//         await buildServer();
//     }



// }

// build();