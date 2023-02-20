const fs = require("fs");
const Parser = require("./Parser");

class BluePrint {
  constructor(source, dst = "src/Docs") {
    this.source = source;
    this.dst = dst;
    this.template = "svelte-blueprint/templates/Blueprint.svelte";
  }

  setTemplate(template) {
    if (template) this.template = template;
  }

  doFiles(files = [], cb = () => {}) {
    files
      .filter((file) => file.split(".").pop().toLocaleLowerCase() === "svelte")
      .map(this.parseComponents, this)
      .forEach(this.createDocs, this);
    cb();
  }

  doDir() {
    const files = fs.readdirSync(this.source);
    this.doFiles(files);
  }

  parseComponents(file) {
    const filePath = `${this.source}/${file}`;
    const data = fs.readFileSync(filePath, "utf8");

    const parser = new Parser(data);
    return {
      name: file.split(".")[0],
      ...parser.parse(),
    };
  }

  calculateRelative() {
    return this.dst.split("/").reduce((ac, _) => `${ac}../`, "") + this.source;
  }

  createDocs(component) {
    const data = `<script>
import Blueprint from '${this.template}'
import ${component.name} from '${this.calculateRelative()}/${
      component.name
    }.svelte'
</script>
    
<Blueprint title='${component.name}' code="${component.example}" >
    <p slot='description'>
        ${component.description}
    </p>
    <table slot='props'>
        <thead>
            <tr>
                <th>Name</th>
                <th>Type</th>
                <th>Default</th>
                <th>Description</th>
            </tr>
        </thead>
        <tbody>
        ${component.props.reduce(
          (ac, props) => `  ${ac}
            <tr>
                <td>${props.name}</td>
                <td>${props.type ? props.type : "-"}</td>
                <td>${props.default ? props.default : "-"}</td>
                <td>${props.description ? props.description : "-"}</td>
            </tr>
        `,
          ""
        )}
        </tbody>
    </table>

    <div slot='example' >
        ${component.example}
    </div>
</Blueprint>
        `;
    fs.writeFileSync(`${this.dst}/${component.name}.svelte`, data);
  }
}

module.exports = BluePrint;
