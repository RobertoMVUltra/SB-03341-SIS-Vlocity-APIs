<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>BanInsGetPackagesIL</name>
   <tag></tag>
   <elementGuidId>4a41c08a-b043-44c6-88cf-6100a5c74a41</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>00DDG000009TCI1!AR0AQNNfOToglZ2cXCn.u33qwjpV5uC.N_OD_QQZjJKtu5WZAWAceT4YAobUes9oWjn2md7YqEisdmoVUIUlxFzYXi5hLIfG</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{  \n    \&quot;type\&quot;: \&quot;${type}\&quot;,  \n    \&quot;data\&quot;: {\n    \&quot;insuredAge\&quot;: \&quot;${insuredAge}\&quot;,\n    \&quot;smoke\&quot;: \&quot;${smoke}\&quot;,\n    \&quot;gender\&quot;: \&quot;${gender}\&quot;,\n    \&quot;insuredAmount\&quot;: \&quot;${insuredAmount}\&quot;\n}\n} &quot;,
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
      <webElementGuid>5854bc6f-4ffc-4156-a528-700f6391cb9b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.TOKEN}</value>
      <webElementGuid>0b907ddb-e6db-434d-8a88-652e23cfc6a5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://segbanortesfi--qa.sandbox.my.salesforce.com/services/apexrest/vlocity_ins/v1/integrationprocedure/BanIns_GetPackagesIL</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'lifeFormula'</defaultValue>
      <description></description>
      <id>90ad9546-e95b-4d67-b771-d21d95beb8c9</id>
      <masked>false</masked>
      <name>type</name>
   </variables>
   <variables>
      <defaultValue>'25'</defaultValue>
      <description></description>
      <id>a5317457-70e9-4503-87d5-5ba2ee9904f5</id>
      <masked>false</masked>
      <name>insuredAge</name>
   </variables>
   <variables>
      <defaultValue>'false'</defaultValue>
      <description></description>
      <id>2ffa366c-c1ff-4997-8392-6ab9d7a2cb65</id>
      <masked>false</masked>
      <name>smoke</name>
   </variables>
   <variables>
      <defaultValue>'F'</defaultValue>
      <description></description>
      <id>aeeed680-da8e-4dbd-9e70-2916f36ddbde</id>
      <masked>false</masked>
      <name>gender</name>
   </variables>
   <variables>
      <defaultValue>'150000'</defaultValue>
      <description></description>
      <id>773fd1ed-c407-4570-9883-c9f162fb30c0</id>
      <masked>false</masked>
      <name>insuredAmount</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
