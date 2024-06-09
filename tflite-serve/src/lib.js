const { Interpreter } = require("node-tflite");
const modelData = fs.readFileSync("../model.tflite");
const interpreter = new Interpreter(modelData);

exports.predict = (msg) => {
    interpreter.allocateTensors();
    interpreter.inputs[0].copyFrom(msg);
    interpreter.invoke();
    result;
    interpreter.outputs[0].copyTo(result);
    return result;
}