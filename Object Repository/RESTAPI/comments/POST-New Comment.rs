<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST-New Comment</name>
   <tag></tag>
   <elementGuidId>8df05e34-4fe5-4b47-9ee7-38ea7ed03812</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;postId\&quot;: 100,\n  \&quot;name\&quot;: \&quot;madina\&quot;,\n  \&quot;email\&quot;: \&quot;testing@baggy.ac.id\&quot;,\n  \&quot;body\&quot;: \&quot;testing\\nsedang test\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>75d4f0bc-5a35-441b-aed5-d528d5bceb6a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/comments/</restUrl>
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

WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)

WS.verifyElementPropertyValue(response, 'postId', 100)
WS.verifyElementPropertyValue(response, 'name', 'madina')
WS.verifyElementPropertyValue(response, 'email', 'testing@baggy.ac.id')
WS.verifyElementPropertyValue(response, 'body', 'testing\nsedang test')
WS.verifyElementPropertyValue(response, 'id', 501)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
