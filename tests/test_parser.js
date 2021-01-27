const Parser = require('../src/Parser')
const expect = require('chai').expect
const fs = require('fs')

describe('Check file parser',  () => {
    const data = fs.readFileSync('./examples/Button.svelte', 'utf8')
    const parser = new Parser(data)
    const parsed = parser.parse()
    
    it('should match description', () => {
        expect(parsed.description).to.equal('Componente de botón')
    })

    it('should match props', () => {
        expect(parsed.props[0].name).to.equal('text')
        expect(parsed.props[0].type).to.equal('string')
        expect(parsed.props[0].default).to.equal('Button')
        expect(parsed.props[0].description).to.equal('Variable del texto en el botón')

        expect(parsed.props[1].name).to.equal('index')
        expect(parsed.props[1].type).to.equal('number')
        expect(parsed.props[1].default).to.equal('5')
        expect(parsed.props[1].description).to.equal('Número de botón')

        expect(parsed.props[2].name).to.equal('data')
        expect(parsed.props[2].type).to.equal('object')
        expect(parsed.props[2].default).to.equal('{}')
        expect(parsed.props[2].description).to.equal('')
    })

    it('should match example', () => {
        expect(parsed.example).to.equal('\n<Button  />\n<Button text=\'One\' />\n<Button text=\'Two\' />\n<Button text=\'Three\' />\n ')
    })
})