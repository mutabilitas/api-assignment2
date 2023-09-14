<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET-List Todos</name>
   <tag></tag>
   <elementGuidId>0cb72874-b030-459e-8dca-923416f300fb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/todos</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, '[1].userId', 1)
WS.verifyElementPropertyValue(response, '[1].id', 2)
WS.verifyElementPropertyValue(response, '[1].title', &quot;quis ut nam facilis et officia qui&quot;)
WS.verifyElementPropertyValue(response, '[1].completed', false)

WS.verifyElementPropertyValue(response, '[3].userId', 1)
WS.verifyElementPropertyValue(response, '[3].id', 4)
WS.verifyElementPropertyValue(response, '[3].title', &quot;et porro tempora&quot;)
WS.verifyElementPropertyValue(response, '[3].completed', true)

WS.verifyElementPropertyValue(response, '[34].userId', 2)
WS.verifyElementPropertyValue(response, '[34].id', 35)
WS.verifyElementPropertyValue(response, '[34].title', &quot;repellendus veritatis molestias dicta incidunt&quot;)
WS.verifyElementPropertyValue(response, '[34].completed', true)

WS.verifyElementPropertyValue(response, '[38].userId', 2)
WS.verifyElementPropertyValue(response, '[38].id', 39)
WS.verifyElementPropertyValue(response, '[38].title', &quot;doloremque quibusdam asperiores libero corrupti illum qui omnis&quot;)
WS.verifyElementPropertyValue(response, '[38].completed', false)

WS.verifyElementPropertyValue(response, '[199].userId', 10)
WS.verifyElementPropertyValue(response, '[199].id', 200)
WS.verifyElementPropertyValue(response, '[199].title', 'ipsam aperiam voluptates qui')
WS.verifyElementPropertyValue(response, '[199].completed', false)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
