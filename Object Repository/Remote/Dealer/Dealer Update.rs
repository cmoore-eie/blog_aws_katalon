<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Dealer Update</name>
   <tag></tag>
   <elementGuidId>7fad1a2b-548f-4163-91e0-123418ce82cb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;ItemUUID\&quot;: \&quot;${uuid}\&quot;,\n    \&quot;Code\&quot;: \&quot;${code}\&quot;,\n    \&quot;Name\&quot;: \&quot;${name}\&quot;,\n    \&quot;Locations\&quot;: [{\&quot;Name\&quot;: \&quot;New Location 1\&quot;, \n                   \&quot;ItemType\&quot;: \&quot;DEALER_LOCATION#da6ee8a3-6ee6-416b-82cd-38e4aa1b4d0e\&quot;}]\n}&quot;,
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
   </httpHeaderProperties>
   <katalonVersion>7.7.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://vxymc6j9r0.execute-api.us-east-1.amazonaws.com/development/dealer</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'UUID31'</defaultValue>
      <description></description>
      <id>0863c04b-b36f-45e1-b314-aceba04443bc</id>
      <masked>false</masked>
      <name>uuid</name>
   </variables>
   <variables>
      <defaultValue>'d1'</defaultValue>
      <description></description>
      <id>d8668438-e3e8-4014-a59b-d0eaed78f09e</id>
      <masked>false</masked>
      <name>code</name>
   </variables>
   <variables>
      <defaultValue>'Dealer Name 1 Changed'</defaultValue>
      <description></description>
      <id>24c38b17-a876-4cd0-856f-9229a2288292</id>
      <masked>false</masked>
      <name>name</name>
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
