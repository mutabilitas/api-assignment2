<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET-List Comments</name>
   <tag></tag>
   <elementGuidId>cec738d8-b1fa-4f5c-b3a4-c1b3a535e4f9</elementGuidId>
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
   <restUrl>https://jsonplaceholder.typicode.com/comments</restUrl>
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

WS.verifyElementPropertyValue(response, '[2].postId', 1)
WS.verifyElementPropertyValue(response, '[2].id', 3)
WS.verifyElementPropertyValue(response, '[2].name', &quot;odio adipisci rerum aut animi&quot;)
WS.verifyElementPropertyValue(response, '[2].email', 'Nikita@garfield.biz')
WS.verifyElementPropertyValue(response, '[2].body', 'quia molestiae reprehenderit quasi aspernatur\naut expedita occaecati aliquam eveniet laudantium\nomnis quibusdam delectus saepe quia accusamus maiores nam est\ncum et ducimus et vero voluptates excepturi deleniti ratione')

WS.verifyElementPropertyValue(response, '[5].postId', 2)
WS.verifyElementPropertyValue(response, '[5].id', 6)
WS.verifyElementPropertyValue(response, '[5].name', 'et fugit eligendi deleniti quidem qui sint nihil autem')
WS.verifyElementPropertyValue(response, '[5].email', 'Presley.Mueller@myrl.com')
WS.verifyElementPropertyValue(response, '[5].body', 'doloribus at sed quis culpa deserunt consectetur qui praesentium\naccusamus fugiat dicta\nvoluptatem rerum ut voluptate autem\nvoluptatem repellendus aspernatur dolorem in')

WS.verifyElementPropertyValue(response, '[55].postId', 12)
WS.verifyElementPropertyValue(response, '[55].id', 56)
WS.verifyElementPropertyValue(response, '[55].name', 'et dolorem corrupti sed molestias')
WS.verifyElementPropertyValue(response, '[55].email', 'Vince_Crist@heidi.biz')
WS.verifyElementPropertyValue(response, '[55].body', 'cum esse odio nihil reiciendis illum quaerat\nest facere quia\noccaecati sit totam fugiat in beatae\nut occaecati unde vitae nihil quidem consequatur')

WS.verifyElementPropertyValue(response, '[192].postId', 39)
WS.verifyElementPropertyValue(response, '[192].id', 193)
WS.verifyElementPropertyValue(response, '[192].name', 'quia velit nostrum eligendi voluptates')
WS.verifyElementPropertyValue(response, '[192].email', 'Alena@deron.name')
WS.verifyElementPropertyValue(response, '[192].body', 'laudantium consequatur sed adipisci a\nodit quia necessitatibus qui\nnumquam expedita est accusantium nostrum\noccaecati perspiciatis molestiae nostrum atque')

WS.verifyElementPropertyValue(response, '[328].postId', 66)
WS.verifyElementPropertyValue(response, '[328].id', 329)
WS.verifyElementPropertyValue(response, '[328].name', 'quasi beatae sapiente voluptates quo temporibus')
WS.verifyElementPropertyValue(response, '[328].email', 'Corbin.Hilll@modesto.biz')
WS.verifyElementPropertyValue(response, '[328].body', 'nulla corrupti delectus est cupiditate explicabo facere\nsapiente quo id quis illo culpa\nut aut sit error magni atque asperiores soluta\naut cumque voluptatem occaecati omnis aliquid')

WS.verifyElementPropertyValue(response, '[439].postId', 88)
WS.verifyElementPropertyValue(response, '[439].id', 440)
WS.verifyElementPropertyValue(response, '[439].name', 'adipisci sapiente libero beatae quas eveniet')
WS.verifyElementPropertyValue(response, '[439].email', 'Adolf.Russel@clark.ca')
WS.verifyElementPropertyValue(response, '[439].body', 'ut nam ut asperiores quis\nexercitationem aspernatur eligendi autem repellendus\nest repudiandae quisquam rerum ad explicabo suscipit deserunt eius\nalias aliquid eos pariatur rerum magnam provident iusto')

WS.verifyElementPropertyValue(response, '[499].postId', 100)
WS.verifyElementPropertyValue(response, '[499].id', 500)
WS.verifyElementPropertyValue(response, '[499].name', 'ex eaque eum natus')
WS.verifyElementPropertyValue(response, '[499].email', 'Emma@joanny.ca')
WS.verifyElementPropertyValue(response, '[499].body', 'perspiciatis quis doloremque\nveniam nisi eos velit sed\nid totam inventore voluptatem laborum et eveniet\naut aut aut maxime quia temporibus ut omnis')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
