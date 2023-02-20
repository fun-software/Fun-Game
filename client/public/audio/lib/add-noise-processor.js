// addNoiseProcessor.js
class addNoiseProcessor extends AudioWorkletProcessor {
  static get parameterDescriptors() {
    return [
      {name: 'noiseAmount', defaultValue: 0, minValue: 0, maxValue: 1}
    ];
  }

  clamp(parameterValue, paramMin, paramMax) {
    return Math.min(Math.max(parameterValue, paramMin), paramMax);
  }

  equalPowerCrossfadeCoefficients(parameterValue) {
    // takes a parameterValue between [0, 1] and returns
    // coefficents for mixing two signals, equally.
    // ex: dry and wet signals of an effect.
    parameterValue = this.clamp(parameterValue, 0, 1);
    const aCoeff = Math.cos(parameterValue * Math.PI*0.5);
    const bCoeff = Math.sin(parameterValue * Math.PI*0.5);
    console.log("aCoeff: ", aCoeff, "bCoeff: ", bCoeff);
    return [aCoeff, bCoeff];
  } 

  process(inputs, outputs, parameters) {
    const input = inputs[0];
    const output = outputs[0];
    const noiseDryWetPercentage = parameters.noiseAmount[0];
    const [dryCoeff, wetCoeff] = this.equalPowerCrossfadeCoefficients(noiseDryWetPercentage);
    
    for (let channel = 0; channel < input.length; ++channel) {
      const inputChannel = input[channel];
      const outputChannel = output[channel];

      for (let i = 0; i < inputChannel.length; i++) {
        let drySignal = inputChannel[i];
        let wetSignal = drySignal * (Math.random() * 2 - 1);
        outputChannel[i] = drySignal * dryCoeff + wetSignal * wetCoeff;
      }
    };
    
    return true; // dont allow the node to be gc'd when audio is not flowing thru
  }
}

registerProcessor("add-noise-processor", addNoiseProcessor);