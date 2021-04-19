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

There is definitely no official IoT discord server, but there might be an unofficial one that I don't know about. The discussion boards on itslearning were supposed to take that role.

And yes, memory consumption (and cpu) could be measured as well. The test harness actually does this already. 
But for this exercise it is less relevant (unless you see weird stuff happening).
