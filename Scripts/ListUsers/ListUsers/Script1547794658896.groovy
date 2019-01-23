import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.sun.xml.internal.org.jvnet.mimepull.DataFile as DataFile
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequestAndVerify(findTestObject('User/LIST USERS'))

def slurper= new groovy.json.JsonSlurper()

def result1= slurper.parseText(response.getResponseBodyContent())

def firstNameValue=result1.firstNamePath
GlobalVariable.firstName=firstNameValue

def lastNameValue=result1.lastNamePath
GlobalVariable.lastName=lastNameValue

WS.verifyElementPropertyValue(response, firstNamePath, GlobalVariable.firstName)

println("DataFile  : " )

WS.verifyElementPropertyValue(response, lastNamePath, GlobalVariable.lastName)

WS.verifyResponseStatusCode(response, 200)

def val = response.getStatusCode()

println('value of status code is : ' + val)

