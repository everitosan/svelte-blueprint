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
        const dirtyNameVal = prop.split('let').pop().split('=')
        const [name, val] = dirtyNameVal.map(el => el.trim())
        const res = {}
        res.name = name
        if(val) {
            res.default = val
            res.type = typeof val
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