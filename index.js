const yargs = require('yargs')
const fs = require('fs')
const SvelteDocs = require('./src/SvelteDocs')


let sourcePath='src/Components';
let dstPath = 'Blueprints'
let wkin = false

const argv = yargs
    .option('source', {
        description: 'Source path of components',
        alias: 's',
        type: 'string'
    })
    .option('dst', {
        description: 'Destination path for docs',
        alias: 'd',
        type: 'string'
    })
    .option('watch', {
        description: 'Should watch for changes in source',
        alias: 'w',
        type: 'boolean'
    })
    .option('template', {
        description: 'Path of a template for the output page',
        alias: 't',
        type: 'string'
    })
    .argv

if (argv.source) sourcePath = argv.source
if (argv.dst) dstPath = argv.dst
dstPath = dstPath + '/Pages'

if(!fs.existsSync(sourcePath)) throw new Error(`The path ${sourcePath} does not exists`)
if(!fs.existsSync(dstPath)) fs.mkdirSync(dstPath, {recursive: true})

const sd = new SvelteDocs(sourcePath, dstPath)
if(argv.template) sd.setTemplate(argv.template)

// Create the first time
sd.doDir()

// Watch for changes in source
if(argv.watch) {
    fs.watch(sourcePath, { encoding: 'utf8' }, (eventType, filename) => {
        if (eventType === 'change' && !wkin) {
            wkin = true
            sd.doFiles([filename],  () => { wkin=false })
        }
    })
}
