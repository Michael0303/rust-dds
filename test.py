import messages_pb2

joints = messages_pb2.Joint
robot = messages_pb2.Robot

name = "HeadYaw"

setattr(joints, name, 0.1) #joints.HeadYaw = 0.1

print(joints.HeadYaw)