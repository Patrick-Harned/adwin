# Rust Implementation of the AWDIN drift detection algorithm

From https://riverml.xyz/dev/api/drift/ADWIN/:

"ADWIN (ADaptive WINdowing) is a popular drift detection method with mathematical guarantees. ADWIN efficiently keeps a variable-length window of recent items; such that it holds that there has no been change in the data distribution. This window is further divided into two sub-windows 
 used to determine if a change has happened. ADWIN compares the average of 
 and to confirm that they correspond to the same distribution. Concept drift is detected if the distribution equality no longer holds. Upon detecting a drift, 
 is replaced by and a new is initialized. ADWIN uses a significance value 
 to determine if the two sub-windows correspond to the same distribution."
 
 
 ## Useage
 
 Accepts an integer stream from stdin and emits a message when drift is found. 
 
 ## Tests
 
 We should find drift under the following conditions:
 
 1. Monotonically increasing stream values
 2. Sudden increases
 
 We should not find drift if:
 
 1. Values are more or less in the same range.
 
 This can be validated from a unix shell. 
 
No drift:

`for i in $(seq 1 100); echo 1 | ./main`

Drift:

`seq 1 100 | ./main`

![image](https://user-images.githubusercontent.com/73145124/224585186-63b45a88-81d1-4694-938f-e258ef05d4be.png)
