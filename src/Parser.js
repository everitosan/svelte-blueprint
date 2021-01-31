const { parsed } = require("yargs");

const descRegex=/<!--D (.+) -->/;
const exampleRegex=/<!--E((\n.*?)*)-->/;
const propsRegex=/export let (.+)\n/g;

class Parser {
    
    constructor(data) {
        this.data = data
        this.response = {
            description: 'No description',
            props: [],
            example: 'No example'
        }
    }

    deconstuctProp(prop) {
        const res = {}
        const dirtyNameVal = prop.split('let').pop().split('=')
        const [name, val] = dirtyNameVal.map(el => el.trim())
        
        res.name = name.split('//')[0].trim()
        if(val) {
            const nval = val.split('//')[0].trim()
            // Check type of default
            try {
                const obj = JSON.parse(nval)
                const isArray = obj instanceof Array
                res.type = (isArray ? 'array' :  typeof obj)
            }
            catch {
                res.type = typeof nval
            }
            // clean the default value
            res.default = nval.replace(/'/gi,'')
        }

        const description = prop.split('//')

        if(description.length > 1) {
            res.description = description.pop().trim()
        } else {
            res.description = ''
        }

        return res
    }

    parse() {
        // Get description
        const desRes = descRegex.exec(this.data)
        if(desRes) this.response.description = desRes[1]
        
        // Get example
        const exRes = exampleRegex.exec(this.data)
        if(exRes) this.response.example = exRes[1]
         // Get proprties
        const propsRes = this.data.match(propsRegex)
        if(propsRes) {
            this.response.props = propsRes.map(prop => {
                return this.deconstuctProp(prop)
            })
        }
        return this.response

    }
}

module.exports=Parser