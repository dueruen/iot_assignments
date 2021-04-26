```bash
mosquitto
mosquitto_sub -v -t "func/+/+"

make MqttFunc
make MqttMovingAverage
make MqttSignalGenerator

##Log to files on Unix systems
java -classpath ".:javasysmon-0.3.5.1.jar" TestHarness test "mosquitto_sub -v -t "func/+/+""

//On windows

```
