<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>LIST USERS</name>
   <tag></tag>
   <elementGuidId>3b4b10dc-3ebb-4f6c-b8cb-680114892d31</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://reqres.in/api/users?page=2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.firstNamePath</defaultValue>
      <description></description>
      <id>45c871e5-cf73-4c9d-9c08-4e64482f7f5e</id>
      <masked>false</masked>
      <name>firstNamePath</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.firstName</defaultValue>
      <description></description>
      <id>f388c090-4e5a-4f2a-892a-d02c4ca381d0</id>
      <masked>false</masked>
      <name>firstName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lastnamePath</defaultValue>
      <description></description>
      <id>adf7d746-2926-4fa1-86a0-0fa85aecb5db</id>
      <masked>false</masked>
      <name>lastNamePath</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.lastName</defaultValue>
      <description></description>
      <id>e7fe4a7d-d4d0-4303-8bb6-f1efbc6ab354</id>
      <masked>false</masked>
      <name>lastName</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementPropertyValue(response, 'data[1].first_name', &quot;Charles&quot;)
WS.verifyElementPropertyValue(response, 'page', 2)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
