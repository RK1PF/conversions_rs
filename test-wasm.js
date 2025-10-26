const { 
    convert_length_wasm, 
    convert_weight_wasm, 
    convert_temperature_wasm,
    convert_volume_wasm,
    convert_time_wasm,
    convert_area_wasm,
    get_supported_units
} = require('./pkg/nodejs/conversions_rs.js');

console.log('🔄 Testing Conversions RS WASM Module\n');

// Test length conversion
console.log('📏 Length Conversions:');
const lengthResult = convert_length_wasm(100, "ft", "m");
if (lengthResult.success) {
    console.log(`✅ 100 feet = ${lengthResult.value.toFixed(4)} meters`);
} else {
    console.log(`❌ Error: ${lengthResult.error}`);
}

// Test weight conversion
console.log('\n⚖️ Weight Conversions:');
const weightResult = convert_weight_wasm(10, "kg", "lb");
if (weightResult.success) {
    console.log(`✅ 10 kg = ${weightResult.value.toFixed(4)} pounds`);
} else {
    console.log(`❌ Error: ${weightResult.error}`);
}

// Test temperature conversion
console.log('\n🌡️ Temperature Conversions:');
const tempResult = convert_temperature_wasm(25, "C", "F");
if (tempResult.success) {
    console.log(`✅ 25°C = ${tempResult.value.toFixed(2)}°F`);
} else {
    console.log(`❌ Error: ${tempResult.error}`);
}

// Test volume conversion
console.log('\n🫗 Volume Conversions:');
const volumeResult = convert_volume_wasm(5, "l", "gal");
if (volumeResult.success) {
    console.log(`✅ 5 liters = ${volumeResult.value.toFixed(4)} US gallons`);
} else {
    console.log(`❌ Error: ${volumeResult.error}`);
}

// Test time conversion
console.log('\n⏰ Time Conversions:');
const timeResult = convert_time_wasm(120, "min", "h");
if (timeResult.success) {
    console.log(`✅ 120 minutes = ${timeResult.value.toFixed(2)} hours`);
} else {
    console.log(`❌ Error: ${timeResult.error}`);
}

// Test area conversion
console.log('\n🏞️ Area Conversions:');
const areaResult = convert_area_wasm(10000, "m²", "ha");
if (areaResult.success) {
    console.log(`✅ 10000 m² = ${areaResult.value.toFixed(2)} hectares`);
} else {
    console.log(`❌ Error: ${areaResult.error}`);
}

// Test error handling
console.log('\n❌ Error Handling:');
const errorResult = convert_length_wasm(100, "ft", "invalid_unit");
if (!errorResult.success) {
    console.log(`✅ Properly caught error: ${errorResult.error}`);
} else {
    console.log(`❌ Should have failed but didn't`);
}

// Test supported units
console.log('\n📋 Supported Units:');
const lengthUnits = get_supported_units("length");
console.log(`Length units: ${lengthUnits.join(', ')}`);

const tempUnits = get_supported_units("temperature");
console.log(`Temperature units: ${tempUnits.join(', ')}`);

console.log('\n🎉 All tests completed!');