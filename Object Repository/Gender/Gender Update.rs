<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Gender Update</name>
   <tag></tag>
   <elementGuidId>b2dc8f58-118d-421e-8910-b6414da9e4d4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;ItemUUID\&quot;: \&quot;${uuid}\&quot;,\n    \&quot;Code\&quot;: \&quot;${code}\&quot;,\n    \&quot;Name\&quot;: \&quot;${name}\&quot;,\n    \&quot;BranchType\&quot;: \&quot;${branchtype}\&quot;\n}&quot;,
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
   <restUrl>http://127.0.0.1:3000/genderupdate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'UUID11'</defaultValue>
      <description></description>
      <id>0863c04b-b36f-45e1-b314-aceba04443bc</id>
      <masked>false</masked>
      <name>uuid</name>
   </variables>
   <variables>
      <defaultValue>'o'</defaultValue>
      <description></description>
      <id>d8668438-e3e8-4014-a59b-d0eaed78f09e</id>
      <masked>false</masked>
      <name>code</name>
   </variables>
   <variables>
      <defaultValue>'Gender Other Changed'</defaultValue>
      <description></description>
      <id>24c38b17-a876-4cd0-856f-9229a2288292</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'2020-01-06'</defaultValue>
      <description></description>
      <id>c0c8ec65-6d24-418f-a228-ffb70d66ab96</id>
      <masked>false</masked>
      <name>effectivedate</name>
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
