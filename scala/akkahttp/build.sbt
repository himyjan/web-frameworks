name := "server"
scalaVersion := "3.3.1"

libraryDependencies ++= Seq(
  "com.typesafe.akka" %% "akka-http" % "10.5.2",
  "com.typesafe.akka" %% "akka-stream" % "[2.8,2.9)"
)

enablePlugins(JavaAppPackaging)
